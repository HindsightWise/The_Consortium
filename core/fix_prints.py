import os
import glob
import re

def fix_prints():
    rs_files = glob.glob('consortium_engine/src/**/*.rs', recursive=True)
    for filepath in rs_files:
        with open(filepath, 'r') as f:
            lines = f.readlines()
        
        new_lines = []
        for line in lines:
            if 'println!("{}", msg);' in line:
                new_lines.append(line)
            else:
                line = re.sub(r'\bprintln!', 'crate::c_print!', line)
                line = re.sub(r'\beprintln!', 'crate::c_print!', line)
                new_lines.append(line)
        
        with open(filepath, 'w') as f:
            f.writelines(new_lines)

if __name__ == '__main__':
    fix_prints()
