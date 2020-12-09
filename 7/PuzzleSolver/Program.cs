using System;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.IO;
using System.Linq;
namespace PuzzleSolver
{
    class Program
    {
        static List<KeyValuePair<string, List<KeyValuePair<string, int>>>> rules = new List<KeyValuePair<string, List<KeyValuePair<string, int>>>>();
        async static Task<List<KeyValuePair<string, List<KeyValuePair<string, int>>>>> parseRules() {

            string readData = await File.ReadAllTextAsync(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

            var output = new List<KeyValuePair<string, List<KeyValuePair<string, int>>>>();

            foreach (string rule in readData.Split('\n'))
            {
                if (rule == "")
                {
                    continue;
                }
                string ruleSubject = rule.Substring(0, rule.IndexOf("bags") - 1);

                int regardStart = rule.IndexOf("contain") + "contain".Length;
                int regardEnd = rule.IndexOf('.');

                string ruleRegards = rule.Substring(regardStart, regardEnd - regardStart); //calculate length by subtracktion
                var regards = new List<KeyValuePair<string, int>>();
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
                    regards.Add(new KeyValuePair<string,int>(bagColor, containedBags));
                    // Console.WriteLine($"\"{iteration}\",\"{bagColor}\", \"{ruleSubject}\"");
                    
                }
                output.Add(new KeyValuePair<string, List<KeyValuePair<string, int>>>(ruleSubject, regards));
                
            }
            return output;
        }


        async static Task Main(string[] args)
        {
            rules = await parseRules();

            // part 1
            BagNode RootNode = new BagNode("shiny gold");
            int changes = 0;
            int iteration = 1;
            do {
                changes = 0;
                foreach(var rule in rules) {
                    // Console.WriteLine(rule);
                    foreach (var regard in rule.Value)
                    {   
                        if (RootNode.Append(regard.Key, rule.Key))
                        {
                            changes++;
                        }
                    }
                }

                Console.WriteLine($"koniec serii {iteration} z {changes} zmianami");
                iteration++;

            }
            while(changes > 0);
            var parentNodesNames = RootNode.GetNodesContaining();
            Console.WriteLine($"liczba kolorow toreb ktore moga sobie na chillku zawierac zlota torebeczke  to {parentNodesNames.Count}");

            //part 2
            Console.WriteLine($"part 2 na loozaku {await CalcBags("shiny gold", 1) - 1}"); // -1 because im not counting the shiny gold bag in

            

        }

        static async Task<int> CalcBags(string bagName, int bagQuantity) { 
            int bags = 1;

            foreach(var rule in rules) {
                if(rule.Key==bagName) {
                    foreach(var regard in rule.Value) {
                        bags += await CalcBags(regard.Key, regard.Value); 
                    }
                }
            }
              
            return bags * bagQuantity;
        }
    }

    class BagNode
    {
        private string nodeName;
        private int nodeCount;
        public string NodeName { get { return nodeName; } }
        private List<BagNode> parentNodes = new List<BagNode>();
        public BagNode(string nodeName, int nodeCount = 1)
        {
            this.nodeName = nodeName;
            this.nodeCount = nodeCount;
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
                
                return this.appendParent(parentName);
            }
            foreach (BagNode node in parentNodes)
            {
                if (node.Append(nodeName, parentName))
                {
                    return true;
                }
            }
            return false;
        }
        public HashSet<string> GetNodesContaining(bool root = true) {
            HashSet<string> parentNodesNames = new HashSet<string>();
            // dont count root object to parents
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
