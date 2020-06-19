# TODOs


# Small TODO

 * after persistence is done, persist logs

 * auto prune logs at a certain size

 * Display logs to user on UI

 * "open file" adds to a store of "file to send" list. Display these files in the UI

 * allow sending files from the GUI
 * * Configure destination IP address in the GUI

 * Break things out into a more coherent file structure instead of all in one for the GUI

 * make log messages into a scrollables

# medium TODO

 * Add persistence. Find some cross-platform equivalent of appdata and store a file and/or sqlite DB for data that shouldn't be per-session

 * UI to add and maintain peers. by IP address is probably a start. With persistence, these peers should be remembered.

 * start up, shut down, and view state of transferrous-server from within GUI

# large TODO

 * Upgrade communications for direct file sending to some kind of communication protocol. on connection, each side should present an identity and spew out N commands they have enqueued for the other identity

 * Upgrade file sending. Instead of sending a whole file we should maybe .tar.gz, maybe do some kind of rsync thing, etc.

 * Add public key encryption. On first run, transferrous should create a pubkey and secret. All communications should use pubkey encryption to send a shared secret and then communication happens encrypted by that secret

 * As the UI becomes the "main" way to use transferrous, we should consider breaking out functionality into a library and making a full featured CLI that has all the same functionality as the UI (e.g. instead of a pane to display logs, a -l flag that displays logs, etc.)