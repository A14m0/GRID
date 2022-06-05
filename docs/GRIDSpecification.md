# GRID Protocol Specifications
## Request Model 
GRID (as of now) is designed to be a client/server protocol over TLS. GRID 
requests will all be binary. Minimal plaintext will be included, and all 
requests will be packed in bytecode. 

## Operation Codes
GRID supports a number of operations and response codes.

## Request Header
Each GRID request will be led by a 33-byte header which includes the following
fields:
1. OPCODE (1 byte): Defines the purpose of the request. This includes both request operations as well as server response codes.
2. PATH_SIZE (16 bytes): Defines the length of the incldued path 