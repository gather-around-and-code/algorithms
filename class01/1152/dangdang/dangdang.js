const readline = require('readline');

const rl = readline.createInterface({
	input: process.stdin,
	output: process.stdout
});

rl.on('line', (line) => {
	solution(line);
	rl.close();
});

rl.on('close', () => {
	process.exit();
});


function solution(line) {
	const words = line.trim().split(' ').filter(p => p != null && p != undefined && p != '');
	console.log(words.length);
}