
const fs = require('fs')
fs.readFile('input.txt', (err, data) => {
    if (err) {
        console.error('error reading file!');
        return;
    }
    // pierdolony javascript potrzebuje regex do glupiego newline'a
    let input = data.toString('utf-8').replace(/(?:\r\n|\r|\n)/g, ' ');

    let passportData = input.split('  ');

    let validPasswords = 0;

    for (data of passportData) {
        fields = {}
        for (field of data.split(' ')) {
            let key, value;
            [key, value] = field.split(':');
            fields[key] = value;
        }

        if (fields.byr === undefined || fields.byr < 1920 || fields.byr > 2002) {
            continue;
        }
        if (fields.iyr === undefined || fields.iyr < 2010 || fields.iyr > 2020) {
            continue;
        }
        if (fields.eyr === undefined || fields.eyr.length != 4 || fields.eyr < 2020 || fields.eyr > 2030) {
            continue;
        }


        if (fields.hgt === undefined || !(fields.hgt.endsWith('cm') || fields.hgt.endsWith('in'))) {
            continue;
        }

        let height, unit;
        [_, height, unit] = fields.hgt.split(/(\d+)/);

        if (unit == 'cm' && !validateHeight(height, 150, 193)) {
            continue;
        }
        else if (unit == 'in' && !validateHeight(height, 56, 76)) {
            continue;
        }

        let hexcolorexp = new RegExp(/^#(?:[0-9a-fA-F]{3}){1,2}$/);
        if(fields.hcl === undefined || !hexcolorexp.test(fields.hcl)) {
            continue;
        }

        let colors = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'];
        if(fields.ecl === undefined || !colors.includes(fields.ecl)) {
            continue;
        }
        if(fields.pid === undefined || fields.pid.length != 9) {
            continue;
        }
        validPasswords++;

    }
    console.log(validPasswords);
});


const validateHeight = (val, min, max) => {
    if (val < min || val > max) {
        return false;
    }
    else {
        return true;
    }
};