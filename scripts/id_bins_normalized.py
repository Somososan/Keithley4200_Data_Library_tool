import json
import sys
import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path, PureWindowsPath, WindowsPath

plt.ioff()
scriptdir = sys.argv[1]
scriptdir = PureWindowsPath(scriptdir)
outputdir = sys.argv[2]
outputdir = PureWindowsPath(outputdir)
filename = Path(scriptdir/PureWindowsPath('id_bins_normalized.json'))
with open(filename,encoding="utf-8") as json_file:
    data = json.load(json_file)
    for serie in data:
        for id_data in filter((lambda dat: dat['designator'] == 'Id' ),serie['data']):
            arr = np.array(id_data['data'][0])
            arr = arr/np.average(arr)
            n, bins, patches = plt.hist(arr, bins='auto', density=True, facecolor='g', alpha=0.75)
            plt.ticklabel_format(axis='both',style='sci', scilimits=(0,0))
            plt.title("normalized histogram of: "+serie['title']) 
            filename = Path(outputdir/PureWindowsPath("id_bins_normalized " + serie['title'] + ".png"))
            sys.stderr.write(str(filename) + '\n')
            plt.savefig(filename,dpi=600)
            plt.close()