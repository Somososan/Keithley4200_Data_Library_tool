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
filename = Path(scriptdir/PureWindowsPath('id_versus_time.json'))
with open(filename,encoding="utf-8") as json_file:
    data = json.load(json_file)
    for serie in data:
        id_datalist = list( filter((lambda dat: dat['designator'] == 'Id'),serie['data']) ) 
        t_datalist = list( filter((lambda dat: dat['designator'] == 'T(s)'),serie['data']) )
        id_arr = np.array(id_datalist[0]['data'][0])
        t_arr = np.array(t_datalist[0]['data'][0])

        plt.plot(t_arr,id_arr)
        plt.ticklabel_format(axis='both',style='sci', scilimits=(0,0))
        plt.title("Id versus time of: "+serie['title']) 
        filename = Path(outputdir/PureWindowsPath("id_versus_time " + serie['title'] + ".png"))
        sys.stderr.write(str(filename) + '\n')
        plt.savefig(filename,dpi=600)
        plt.close()