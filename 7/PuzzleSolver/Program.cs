using System;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.IO;
using System.Linq;
namespace PuzzleSolver
{
    class Program
    {
        async static Task Main(string[] args)
        {

            string readData = await File.ReadAllTextAsync(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

            BagNode RootNode = new BagNode("shiny gold");

            int changes = 0;
            int iteration = 1;
            do
            {
                changes = 0;
                Console.WriteLine("iteration " + iteration);
                foreach (string rule in readData.Split('\n'))
                {
                    if (rule == "")
                    {
                        continue;
                    }
                    string ruleSubject = rule.Substring(0, rule.IndexOf("bags") - 1);

                    int regardStart = rule.IndexOf("contain") + "contain".Length;
                    int regardEnd = rule.IndexOf('.');

                    string ruleRegards = rule.Substring(regardStart, regardEnd - regardStart); //calculate length by subvision
                    foreach (string regard in ruleRegards.Split(','))
                    {
                        // every line starts from space so shift it one char further
                        string bag = regard.Substring(1);

                        // skip the iteration if bag does not contain any other bags
                        if (bag == "no other bags")
                        {
                            continue;
                        }

                        // max number of bags inside does not exceed 10 so i can freely just substring first character
                        int containedBags = Convert.ToInt32(bag.Substring(0, 1));

                        // searching for bag should suffice
                        int bagColorEnd = bag.IndexOf("bag");
                        //skip the number of bags and space
                        int bagColorStart = 2;

                        string bagColor = bag.Substring(bagColorStart, bagColorEnd - bagColorStart - 1);

                        // Console.WriteLine($"\"{iteration}\",\"{bagColor}\", \"{ruleSubject}\"");
                        if (RootNode.Append(bagColor, ruleSubject))
                        {
                            changes++;
                        }
                    }


                }
                Console.WriteLine($"koniec serii {iteration} z {changes} zmianami");
                iteration++;

            }
            while (changes > 0);
            var parentNodesNames = RootNode.GetNodesContaining();
            Console.WriteLine($"liczba kolorow toreb ktore moga sobie na chillku zawierac zlota to {parentNodesNames.Count}");
        }
    }

    class BagNode
    {
        private string nodeName;
        public string NodeName { get { return nodeName; } }
        private List<BagNode> parentNodes = new List<BagNode>();
        public BagNode(string nodeName)
        {
            this.nodeName = nodeName;
        }
        private bool appendParent(string parentName)
        {
            // if there exists parent with the same name, just skip it
            if (this.parentNodes.Where(n => n.nodeName == parentName).ToList().Count > 0)
            {
                return false;
            }
            else
            {
                parentNodes.Add(new BagNode(parentName));
                return true;
            }
        }

        public bool Append(string nodeName, string parentName)
        {
            if (this.nodeName == nodeName)
            {
                // Console.WriteLine($"{this.nodeName}     {nodeName}");
                
                return this.appendParent(parentName);
            }
            foreach (BagNode node in parentNodes)
            {
                if (node.Append(nodeName, parentName))
                {
                    // this.appendParent(parentName);
                    return true;
                }
            }
            return false;
        }
        public HashSet<string> GetNodesContaining(bool root = true) {
            HashSet<string> parentNodesNames = new HashSet<string>();
            if(!root) {
                parentNodesNames.Add(this.nodeName);
            }
            foreach(BagNode node in parentNodes) {
                // Console.WriteLine(node.nodeName);
                parentNodesNames.UnionWith(node.GetNodesContaining(false));
                
                            
            }
            return parentNodesNames;
        }

    }

}
