// AppIndex — desktop entry indexer for the launcher.
// Scans ~/.local/share/applications and /usr/share/applications.
pragma Singleton
import QtQuick
import Quickshell.Io

Singleton {
    id: appIndex

    property var entries: []

    Process {
        id: scanner
        command: ["bash", "-c",
            "for f in ~/.local/share/applications/*.desktop /usr/share/applications/*.desktop /var/lib/flatpak/exports/share/applications/*.desktop; do " +
            "[ -f \"$f\" ] || continue; " +
            "grep -q '^Hidden=true' \"$f\" && continue; " +
            "name=$(grep -m1 '^Name=' \"$f\" | cut -d= -f2-); " +
            "exec=$(grep -m1 '^Exec=' \"$f\" | cut -d= -f2-); " +
            "icon=$(grep -m1 '^Icon=' \"$f\" | cut -d= -f2-); " +
            "echo \"$name|$exec|$icon\"; done"]
        stdout: StdioCollector {
            onStreamFinished: {
                const lines = text.trim().split("\n").filter(l => l.length > 0)
                const result = []
                for (const line of lines) {
                    const [name, exec, icon] = line.split("|")
                    result.push({ name, exec, icon })
                }
                appIndex.entries = result
            }
        }
        onExited: reload()
    }

    function reload() {
        scanner.running = true
    }

    Component.onCompleted: reload()

    function results(query, max) {
        if (!query) return entries.slice(0, max || 50)
        const q = query.toLowerCase()
        return entries
            .filter(e => e.name.toLowerCase().includes(q))
            .slice(0, max || 50)
    }
}
