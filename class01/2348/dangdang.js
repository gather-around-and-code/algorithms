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
	const number = parseInt(line);

	for (let i = 1; i <= number; i++) {
		for (let j = 0; j < i; j++) {
			process.stdout.write('*');
		}
		process.stdout.write('\n');
	}
}