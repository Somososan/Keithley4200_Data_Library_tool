import json
import sys
import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path, PureWindowsPath, WindowsPath
import multiprocessing as mp

def serie_single_processor(serie):
    id_datalist = list( filter((lambda dat: dat['designator'] == 'Id'),serie['data']) ) 
    t_datalist = list( filter((lambda dat: dat['designator'] == 'T(s)'),serie['data']) )
    id_arr = np.array(id_datalist[0]['data'][0])
    t_arr = np.array(t_datalist[0]['data'][0])

    plt.plot(t_arr,id_arr)
    plt.ticklabel_format(axis='both',style='sci', scilimits=(0,0))
    plt.title("Id versus time of: " + serie['title']) 
    plt.ylabel('Id(A)')
    plt.xlabel('T(s)')
    
    outputdir = PureWindowsPath(sys.argv[2])
    filename = Path(outputdir/PureWindowsPath("id_versus_time\\" + serie['title'] + ".png"))
    plt.savefig(filename,dpi=600)
    plt.close()

plt.ioff()
scriptdir = sys.argv[1]
scriptdir = PureWindowsPath(scriptdir)

outputdir = sys.argv[2]
outputdir = PureWindowsPath(outputdir + '\\id_versus_time')
if not Path(outputdir).is_dir():
    Path(outputdir).mkdir()

filename = Path(scriptdir/PureWindowsPath('data\\id_versus_time.json'))
with open(filename,encoding="utf-8") as json_file:
    data = json.load(json_file)
    if __name__ == '__main__':
        with mp.Pool(processes=mp.cpu_count())  as p:
            p.map(serie_single_processor, data)
