file1 = open('input.txt', 'r')
lines = file1.readlines()

past_numbers = []

for line in lines:
    number = int(line)
    for past_number in past_numbers:
        for second_past_number in past_numbers:
            sum = number + past_number + second_past_number
            if sum == 2020:
                print("Znaleziono wynik!!!")
                print(number * past_number * second_past_number)
    past_numbers.append(number)