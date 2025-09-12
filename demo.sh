#!/bin/bash
# FalkorDB CLI Demo Script

echo "=== FalkorDB CLI Demo ==="
echo

# Set the CLI path
CLI_PATH="./target/release/falkordb-cli"

echo "1. Creating a sample social network graph..."
$CLI_PATH -g demo --eval "CREATE (alice:Person {name: 'Alice', age: 30, city: 'New York'})" --quiet
$CLI_PATH -g demo --eval "CREATE (bob:Person {name: 'Bob', age: 25, city: 'San Francisco'})" --quiet
$CLI_PATH -g demo --eval "CREATE (charlie:Person {name: 'Charlie', age: 35, city: 'New York'})" --quiet
$CLI_PATH -g demo --eval "MATCH (a:Person {name: 'Alice'}), (b:Person {name: 'Bob'}) CREATE (a)-[:KNOWS {since: 2020}]->(b)" --quiet
$CLI_PATH -g demo --eval "MATCH (a:Person {name: 'Bob'}), (c:Person {name: 'Charlie'}) CREATE (a)-[:KNOWS {since: 2019}]->(c)" --quiet
$CLI_PATH -g demo --eval "MATCH (a:Person {name: 'Alice'}), (c:Person {name: 'Charlie'}) CREATE (a)-[:LIVES_IN {since: 2018}]->(c)" --quiet
echo "Graph created successfully!"
echo

echo "2. Querying all persons..."
$CLI_PATH query demo "MATCH (n:Person) RETURN n.name, n.age, n.city ORDER BY n.age"
echo

echo "3. Finding relationships..."
$CLI_PATH query demo "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name + ' knows ' + b.name AS relationship, r.since AS since"
echo

echo "4. Creating an index for better performance..."
$CLI_PATH create-index demo NODE Person name
echo

echo "5. Showing graph schema..."
$CLI_PATH schema demo
echo

echo "6. Listing indices..."
$CLI_PATH indices demo
echo

echo "7. Profiling a query..."
$CLI_PATH profile demo "MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE a.age > 25 RETURN a.name, b.name"
echo

echo "8. JSON output format..."
$CLI_PATH --format json query demo "MATCH (n:Person) RETURN n.name, n.city LIMIT 2"
echo

echo "9. CSV output format..."
$CLI_PATH --format csv query demo "MATCH (n:Person) RETURN n.name, n.age, n.city"
echo

echo "Demo completed! The graph 'demo' has been created with sample data."
echo "You can explore it further using:"
echo "  $CLI_PATH -g demo interactive"
echo
echo "To clean up the demo graph:"
echo "  $CLI_PATH delete demo"
