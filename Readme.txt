Simple API created in Rust using Rocket for HTTP endpoints and routing and sqlx as database driver. (Please use master branch)
Database used is PostgreSQL.

The application is dockerized and the docker-compose file is included in the root directory.

Steps to run the application

1. Nagivate to root directory (eg. /home/rustapp)
2. docker-compose up -d
3. docker exec -it rustapp_api_server_1 bash
4. cargo test init_db

All APIs are available on http://localhost:3001/ and begin with prefix /api , eg. , http://localhost:3001/api/items
Step 4 is used to initialize the database with values for restaurant menu items and table numbers.

List of available APIs

Postman collection url
https://god.postman.co/run-collection/68da1265684a191a6faf?action=collection%2Fimport

type : GET
url  : /items
Get a list of all items

type : POST
url  : /order
Create new order from posted form data and returns number of rows inserted
Posted data is of type form-data with the following fields
item_qty -> item quantity , format : itemID1_qty1,itemID2_qty2,... (eg. 101_2,102_5)
table -> table ID

type : GET
url : /current_orders/<table>?<item_id> ( eg. http://localhost:3001/api/current_orders/3?item_id=100)
Get all current orders by table id and item id
item_id is optional query parameter , if None , all items returned
The ids being seeded are 100,101,102 (refer /api_server/src/sql/02-seed-items.sql)

type : DELETE
url : /cancel_item/<table>/<item_id>?<quantity> (eg. http://localhost:3001/api/cancel_item/3/101?quantity=1)
Delete an item from an order and returns number of rows affected.
table -> table ID
item_id -> item ID
quantity -> optional , specifies number of items to delete. If not specified or is greater than current number of items in order , all items deleted.


Additional notes:
Time to cook for an item is randomly assigned a number from 5-15 and kept static.
Tables with IDs 1-250 are seeded at the start.
