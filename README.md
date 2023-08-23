[https://github.com/dalsh/wol-proxy](https://github.com/dalsh/wol-proxy)

# Wake On LAN Proxy

Forward magick packets from WAN to LAN

## Usage

`docker run aplagnol/wol-proxy 0.0.0.0:7 255.255.255.255:9`

Listens on all interfaces port 7 UDP and forwards magick packets (if they are valid) to broadcast address port 9 UDP

You will probably need to use `--net host` option for this to be useful

## Mac address whitelisting

You can set a whitelist of mac addresses that can be woken by the proxy by setting the `MAC_ADDRESSES_WHITELIST` value to a comma-separated list of mac addresses.

`docker run aplagnol/wol-proxy 0.0.0.0:7 255.255.255.255:9 -e MAC_ADDRESSES_WHITELIST="11:11:11:11:11:11,22:22:22:22:22:22"`
