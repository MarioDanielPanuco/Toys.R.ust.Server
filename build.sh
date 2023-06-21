cargo update
cargo b --release

nohup ./target/release/web_server &

#This will cause the server to keep running even after you close the SSH session. The output of the server will be saved to a file in the current directory named nohup.out.
#Remember, though, that if you need to shut down the server you will need to manually find its process ID and kill it. You can find the process ID with a command like ps aux | grep web_server, and then use kill -9 <PID> to stop the server. If you find yourself often needing to stop and start the server, or if you want it to start automatically when the system boots, you might be better off creating a systemd service as described in my previous message.