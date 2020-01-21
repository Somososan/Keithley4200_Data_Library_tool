import json
import sys
import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path, PureWindowsPath, WindowsPath
import multiprocessing as mp

def serie_single_processor(serie):
    for id_data in filter((lambda dat: dat['designator'] == 'Id' ),serie['data']):
            arr = np.array(id_data['data'][0])
            fig = plt.figure()
            plt.hist(arr, bins='auto', density=False, facecolor='g', alpha=0.75)
            plt.ticklabel_format(axis='both',style='sci', scilimits=(0,0))
            plt.title("histogram of: "+serie['title']) 
            plt.ylabel('Count')
            plt.xlabel('Id(A)')
            
            outputdir = PureWindowsPath(sys.argv[2])
            filename = outputdir/PureWindowsPath("id_bins\\" + serie['title'] + ".png")
            fig.savefig(filename,dpi=600)
            plt.close(fig=fig)

plt.ioff()
scriptdir = sys.argv[1]
scriptdir = PureWindowsPath(scriptdir)

outputdir = sys.argv[2]
outputdir = PureWindowsPath(outputdir + '\\id_bins')
if not Path(outputdir).is_dir():
    Path(outputdir).mkdir()

filename = Path(scriptdir/PureWindowsPath('data\\id_bins.json'))
with open(filename,encoding="utf-8") as json_file:
    data = json.load(json_file)
    if __name__ == '__main__':
        with mp.Pool(processes=mp.cpu_count())  as p:
            p.map(serie_single_processor, data)
        
        