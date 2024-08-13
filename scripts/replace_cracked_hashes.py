import csv, os, re

regex = "(0x[0-9a-fA-F]{9,10})"
paramlabelscsv = input("ParamLabels.csv: ").replace('"', '')

if not os.path.exists(paramlabelscsv):
    input('Could not find ParamLabels.csv...')
    exit

with open(paramlabelscsv, mode='r') as infile:
    reader = csv.reader(infile)
    paramDict = {rows[0]: rows[1] for rows in reader}

crackedHashes = set()
uncrackedHashes = set()

for root, dirs, files in os.walk(os.pardir):
    if "build" in root:
        continue
    for file in files:
        if file.endswith(".rs"):
            with open(os.path.join(root, file), "r") as f:
                changed = False
                text = f.read()
                
                for hashString in re.findall(regex, text):
                    if len(hashString) == 11:
                        hashString = hashString.replace("0x", "0x0")
                    
                    if hashString in paramDict.keys():
                        changed = True
                        
                        crackedHashes.add(paramDict[hashString])
                        text = text.replace(hashString + "u64", hashString)
                        text = text.replace("Hash40::new_raw(" + hashString, "Hash40::new(\"" + paramDict[hashString] + "\"")

                        withoutLeadingZero = hashString.replace("0x0", "0x")
                        text = text.replace(withoutLeadingZero + "u64", withoutLeadingZero)
                        text = text.replace("Hash40::new_raw(" + withoutLeadingZero, "Hash40::new(\"" + paramDict[hashString] + "\"")

                        #UNCOMMENTING THESE LINES LIKELY CAUSE HDR TO NOT COMPILE AND WILL REQUIRE MANUAL EDITING AFTERWARDS
                        #text = text.replace(hashString, "hash40(\"" + paramDict[hashString] + "\")")
                        #text = text.replace(withoutLeadingZero, "hash40(\"" + paramDict[hashString] + "\")")
                    else:
                        uncrackedHashes.add(hashString)

                if changed:
                    with open(os.path.join(root, file), "w") as f:
                        f.write(text)
                        
        elif file.endswith((".prcxml", ".xml", ".yaml", ".lua")):
            with open(os.path.join(root, file), "r") as f:
                changed = False
                text = f.read()
                
                for hashString in re.findall(regex, text):
                    if len(hashString) == 11:
                        hashString = hashString.replace("0x", "0x0")
                    
                    if hashString in paramDict.keys():
                        changed = True
                        
                        crackedHashes.add(paramDict[hashString])
                        text = text.replace(hashString, paramDict[hashString])

                        withoutLeadingZero = hashString.replace("0x0", "0x")
                        text = text.replace(withoutLeadingZero, paramDict[hashString])
                    else:
                        uncrackedHashes.add(hashString)

                if changed:
                    with open(os.path.join(root, file), "w") as f:
                        f.write(text)
            
                
print(f"Cracked hashes: {crackedHashes}")
print(f"Uncracked hashes: {uncrackedHashes}")
input("Press Enter to exit...")
