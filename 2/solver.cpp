#include <iostream>
#include <fstream>
#include <string>
#include <algorithm>

int main() {
    int compatiblePasswords = 0;
    std::ifstream input("input.txt");
    std::string range, letterWithSemicolon, password;
    while(input >> range >> letterWithSemicolon >> password) {
        char letter = letterWithSemicolon[0];

        auto delimiterPosition = range.find("-");
        // part 1
        /*
        int rangeMin = stoi(range.substr(0, delimiterPosition));
        int rangeMax = stoi(range.substr(delimiterPosition + 1, range.length() - 1));

        auto occurences = std::count(password.begin(), password.end(), letter);

        if(occurences >= rangeMin && occurences <= rangeMax) {
            compatiblePasswords++;
        }*/
        // part 2
        int firstPosition = stoi(range.substr(0, delimiterPosition));
        int secondPostion = stoi(range.substr(delimiterPosition + 1, range.length() - 1));

        char firstLetter = password[firstPosition - 1];
        char secondLetter = password[secondPostion - 1];

        if(firstLetter == letter || secondLetter == letter) {
            if(password[firstPosition - 1] != password[secondPostion - 1]) {
                compatiblePasswords++;
            }
        }
        
    }
    std::cout << compatiblePasswords;
    return 0;
}