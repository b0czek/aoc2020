const fs = require('fs');
const readline = require('readline');

async function processLineByLine() {
    const fileStream = fs.createReadStream('input.txt');

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    });
    let map = [];
    for await (const line of rl) {
        map.push(line);
    }
    return map;
}

const determine = async(hozOffset, verOffset) => {
    let map = await processLineByLine();
    let encounteredTrees = 0;
    let lastPosition = hozOffset;
    for(let i = verOffset; i<map.length; i+=verOffset) {
        const line = map[i];
        let encounteredOccurence = line[lastPosition];
        if(encounteredOccurence == "#") {
            encounteredTrees++;
        }
        if(lastPosition + hozOffset > line.length - 1) {
            lastPosition = lastPosition + hozOffset - line.length;
        }
        else {
            lastPosition += hozOffset;
        }
    }
    return encounteredTrees;
};


const main = async() => {
    slopes = []
    slopes.push(await determine(1,1));
    slopes.push(await determine(3,1));
    slopes.push(await determine(5,1));
    slopes.push(await determine(7,1));
    slopes.push(await determine(1,2));
    console.log(slopes);
    console.log(slopes.reduce((prev, curr) => {
        return prev * curr;
    }));

};
main();
