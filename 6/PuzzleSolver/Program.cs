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
            
            string readData = await File.ReadAllTextAsync(Path.Combine(AppDomain.CurrentDomain.BaseDirectory,"input.txt"));

            List<Group> groups = new List<Group>();
            foreach(string groupData in readData.Split("\n\n")) {
                groups.Add(new Group(groupData));
            }
            // part 1
            int answersSum = 0;
            foreach(Group group in groups) {
                answersSum += group.AnswersCount;
            }
            Console.WriteLine($"Answer count is {answersSum}");        

            // part 2

            answersSum = 0;
            foreach(Group group in groups) {
                answersSum += group.Reevaluate();

            }
            Console.WriteLine($"Answer count #2 is {answersSum}"); 

        }
    }

    class Group {

        List<string> personAnswers = new List<string>();
        HashSet<char> answers = new HashSet<char>();
        public Group(string groupData) {

             foreach(string line in groupData.Split('\n')) {
                 personAnswers.Add(line);
                 foreach(char answer in line) {
                         answers.Add(answer);
                 }
             }
        }

        public int Reevaluate() {
            // sort the list
            var sortedAnswers = personAnswers.OrderBy(a => a.Length);
            // find shortest string 
            string shortestString = sortedAnswers.FirstOrDefault();
            
            List<string> sortedList = sortedAnswers.ToList();

            int matchingAnswers = 0;

            foreach(char answer in shortestString) {
                bool isEglible = true;

                for(int i = 1; i < sortedList.Count(); i++) {
                    if(!sortedList[i].Contains(answer)) {
                        isEglible = false;
                    }
                }

                if(isEglible) {
                    matchingAnswers++;
                }
            }
            return matchingAnswers;

        }

        public int AnswersCount {get { return answers.Count; }}
    }
}