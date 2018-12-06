//To run it: $ node jsclient.js

//Import readline & net modules from std node Library.
const net = require('net');
const readline = require('readline');
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

var client = new net.Socket();

client.connect(3000, '127.0.0.1', function() { //Select port, URL
	console.log('Connected');
});

client.on('data', function(data) {
	console.log('Received: ' + data);
});

client.on('close', function() {
	console.log('Connection closed');
});

rl.on('line', (line) => {
  client.write(line.trim());
});

rl.on('close', () => {
  console.log('Have a great day!');
  client.destroy();
  process.exit(0);
});