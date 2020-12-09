using System;
using System.IO;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.Linq;

namespace PuzzleSolver
{
    class Program
    {
        async static Task Main(string[] args)
        {
            string readData = await File.ReadAllTextAsync(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));
            string[] operations = readData.Split('\n');
            operations = operations.Where(o => o != "").ToArray(); // remove any empty lines

            //part 1
            Console.WriteLine(Instructions.GoThroughInstructions(operations));

            //part 2
            for(int i = 0; i<operations.Length; i++) {
                string[] operationsCopy = Instructions.GetInstructionsCopy(operations);
                string operation = operationsCopy[i];

                string instruction = operation.Split(' ')[0];
                string value = operation.Split(' ')[1]; // not exactly proud there


                if(instruction == "nop") {
                    operationsCopy[i] = $"jmp {value}";
                }
                else if(instruction == "jmp") {
                    operationsCopy[i] = $"nop {value}";
                }
                else {
                    continue;
                }

                if(!Instructions.IsLoopingInfinitely(operationsCopy)) {
                    Console.WriteLine(Instructions.GoThroughInstructions(operationsCopy));
                    break;
                }
            }
        }


    }

    static class Instructions
    {
        public static string[] GetInstructionsCopy(string[] instructions) { // had to do that abomination for the array to be dereferred from og version
            string[] copy = new string[instructions.Length];
            for(int i = 0; i<instructions.Length; i++) {
                copy[i] = instructions[i];
            }
            return copy;
        }
        public static int GoThroughInstructions(string[] operations, bool throwAnErrorIfInfinite = false)
        {
            int i = 0, accumulator = 0;
            List<int> visitedInstructions = new List<int>();
            while (visitedInstructions.Where(v => v == i).ToList().Count == 0 && i < operations.Length)
            {
                visitedInstructions.Add(i);

                string operation = operations[i];

                string[] instructionandvalue = operation.Split(' ');

                string instruction = instructionandvalue[0];
                int value = Convert.ToInt32(instructionandvalue[1]);

                switch (instruction)
                {
                    case "acc":
                        accumulator += value;
                        break;
                    case "jmp":
                        i += value;
                        continue;
                    case "nop":
                        break;
                }
                i++;
            }
            if (throwAnErrorIfInfinite && i < operations.Length)
            {
                throw new Exception("Infinite loop!");
            }


            return accumulator;
        }
        public static bool IsLoopingInfinitely(string[] instructions)
        {
            try
            {
                Instructions.GoThroughInstructions(instructions, true);
                return false;
            }
            catch (Exception)
            {
                return true;
            }
        }
    }

}