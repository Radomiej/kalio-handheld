// Kalio Dev Server — WebSocket hot-reload server for host-first development.
//
// Usage: go run . [watch-dir] [port]
//   watch-dir defaults to "."
//   port      defaults to 8765
//
// The runtime on the handheld (or QEMU) connects via:
//   ws://HOST:8765/ws
// and receives reload events whenever a watched file changes.

package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"sync"
	"time"

	"github.com/fsnotify/fsnotify"
	"github.com/gorilla/websocket"
)

// ── Types ──────────────────────────────────────────────────────────────

type PatchMsg struct {
	Type    string `json:"type"`
	Path    string `json:"path"`
	Content string `json:"content,omitempty"`
	Ts      int64  `json:"ts"`
}

// ── Hub ──────────────────────────────────────────────────────────────────

type hub struct {
	mu      sync.Mutex
	clients map[*websocket.Conn]struct{}
}

func (h *hub) add(c *websocket.Conn) {
	h.mu.Lock()
	h.clients[c] = struct{}{}
	h.mu.Unlock()
}

func (h *hub) remove(c *websocket.Conn) {
	h.mu.Lock()
	delete(h.clients, c)
	h.mu.Unlock()
}

func (h *hub) broadcast(msg []byte) {
	h.mu.Lock()
	defer h.mu.Unlock()
	for c := range h.clients {
		if err := c.WriteMessage(websocket.TextMessage, msg); err != nil {
			log.Printf("write err: %v", err)
		}
	}
}

var (
	conns   = &hub{clients: make(map[*websocket.Conn]struct{})}
	upgrader = websocket.Upgrader{CheckOrigin: func(r *http.Request) bool { return true }}
)

// ── Handlers ─────────────────────────────────────────────────────────────

func wsHandler(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("upgrade: %v", err)
		return
	}
	defer conn.Close()
	conns.add(conn)
	defer conns.remove(conn)
	log.Printf("client connected: %s", conn.RemoteAddr())

	// Keep conn alive — drain incoming (ping/pong handled by gorilla)
	for {
		if _, _, err := conn.ReadMessage(); err != nil {
			break
		}
	}
	log.Printf("client disconnected: %s", conn.RemoteAddr())
}

// ── File watcher ─────────────────────────────────────────────────────────

func watchDir(dir string) {
	w, err := fsnotify.NewWatcher()
	if err != nil {
		log.Fatal(err)
	}
	defer w.Close()

	// Walk and add all subdirs
	_ = filepath.WalkDir(dir, func(path string, d os.DirEntry, err error) error {
		if err == nil && d.IsDir() {
			_ = w.Add(path)
		}
		return nil
	})
	log.Printf("watching: %s", dir)

	// Debounce: collect rapid bursts into one event
	timer := time.NewTimer(0)
	<-timer.C
	pending := map[string]struct{}{}

	for {
		select {
		case ev, ok := <-w.Events:
			if !ok {
				return
			}
			if ev.Has(fsnotify.Write) || ev.Has(fsnotify.Create) {
				pending[ev.Name] = struct{}{}
				timer.Reset(50 * time.Millisecond)
			}
		case <-timer.C:
			for path := range pending {
				sendReload(path)
			}
			pending = map[string]struct{}{}
		case err, ok := <-w.Errors:
			if !ok {
				return
			}
			log.Printf("watcher error: %v", err)
		}
	}
}

func sendReload(path string) {
	content, err := os.ReadFile(path)
	if err != nil {
		log.Printf("read %s: %v", path, err)
		return
	}
	msg := PatchMsg{
		Type:    "reload",
		Path:    path,
		Content: string(content),
		Ts:      time.Now().UnixMilli(),
	}
	data, _ := json.Marshal(msg)
	conns.broadcast(data)
	log.Printf("broadcast reload: %s (%d bytes)", filepath.Base(path), len(content))
}

// ── Main ──────────────────────────────────────────────────────────────────

func main() {
	watchPath := "."
	if len(os.Args) > 1 {
		watchPath = os.Args[1]
	}
	port := "8765"
	if len(os.Args) > 2 {
		port = os.Args[2]
	}

	go watchDir(watchPath)

	http.HandleFunc("/ws", wsHandler)
	http.HandleFunc("/health", func(w http.ResponseWriter, _ *http.Request) {
		fmt.Fprintln(w, "ok")
	})

	addr := ":" + port
	log.Printf("Kalio Dev Server — ws://localhost%s/ws  (watching: %s)", addr, watchPath)
	log.Fatal(http.ListenAndServe(addr, nil))
}
