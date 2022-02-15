#!/usr/bin/python3.9
from os import write
from xml import etree
import xml.etree.ElementTree as ET
import xml.dom.minidom
import pathlib
import sys, getopt

path = pathlib.Path(__file__).parent.parent.resolve()
romfs_src_path = path.joinpath("romfs/source")

if len(sys.argv) < 2:
    print('new_fighter_param.py -t <param_type> -d <default_value> <param_name>')
    sys.exit(2)

try:
    opts, args = getopt.getopt(sys.argv[1::], "ht:d:", ["type=", "default="])
except getopt.GetoptError:
    print('new_fighter_param.py -t <param_type> -d <default_value> <param_name>')
    sys.exit(2)

p_type = None
p_value = None
for o, a in opts:
    if o == 'h':
        print('new_fighter_param.py -t <param_type> -d <default_value> <param_name>')
        sys.exit()
    elif o in ('-t', '--type'):
        p_type = a
    elif o in ('-d', '--default'):
        p_value = a

if p_type == None or p_value == None:
    print('new_fighter_param.py -t <param_type> -d <default_value> <param_name>')
    sys.exit(2)

if len(args) == 0:
    print('new_fighter_param.py -t <param_type> -d <default_value> <param_name>')
    sys.exit(2)

param_name = args[0]


tree = ET.parse(romfs_src_path.joinpath("fighter/common/hdr/param/fighter_param.xml"))
root = tree.getroot()
fighter_param_table = root[0]
for i in range(0, int(fighter_param_table.attrib.get("size"))):
    new_el = ET.Element(p_type, {"hash": param_name})
    new_el.text = p_value
    fighter_param_table[i].append(new_el)

ET.indent(tree, space="  ", level=0)

tree.write(romfs_src_path.joinpath("fighter/common/hdr/param/fighter_param.xml"), encoding="utf-8", xml_declaration=True)