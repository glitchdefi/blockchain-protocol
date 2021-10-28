# SET UP NODE GLITCH

## FULL NODE
1. Call listen address through RPC

``` curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_localListenAddresses"}' http://<ip-node>:<port>/ ```

e.g : ```curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_localListenAddresses"}' http://localhost:9933/```



