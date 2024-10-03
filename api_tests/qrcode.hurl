GET {{target}}/qrcode
HTTP 200

[Asserts]
xpath "string(/html/head/title)" contains "Qr code generator" 