cp "com.kritjo.fastmsg.plist" "$HOME/Library/LaunchAgents/com.kritjo.fastmsg.plist"
launchctl bootstrap gui/"$(id -u)" "$HOME"/Library/LaunchAgents/com.kritjo.fastmsg.plist
