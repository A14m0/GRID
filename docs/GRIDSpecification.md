# GRID Protocol Specifications
## Request Model 
GRID (as of now) is designed to be a client/server protocol over TLS. GRID 
requests will all be binary. Minimal plaintext will be included, and all 
requests will be packed in bytecode. 

## Operation Codes
GRID supports a number of operations and response codes.

Request:

* GET
* PUT
* PNG		Ping
* ABT		Abort
* INF		Info
* TWO		???

Response:
* ROK		Response OK
* GER		General Error
* NOF		Not Found
* BSY		Busy
* RER		Request Error

## Request Header
Each GRID request will be led by a 33-byte header which includes the following
fields:
1. OPCODE (1 byte): Defines the purpose of the request. This includes both request operations as well as server response codes.
2. PATH_SIZE (16 bytes): Defines the length of the incldued path
3. META_DATA_SIZE (16 bytes): Contains any application specific extra data.

The request header is immediately followed by the path and meta_data which are the respective sizes defined in the header.

## Response Header
Each GRID response will be led by a 17 byte header which includes the following fields:
1. OPCODE (1 byte): Defines the purpose of the response.
2. PAYLOAD_SIZE (16 bytes): Size of Payload

The response header is immediately followed by the payload which is the size defined in the header.
