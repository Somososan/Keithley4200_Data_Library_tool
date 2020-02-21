import json
import sys
import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path, PureWindowsPath, WindowsPath
import multiprocessing as mp

def serie_single_processor(serie):
    for t_data in filter((lambda dat: dat['designator'] == 'T(s)' ),serie['data']):
            arr = np.array(t_data['data'][0])
            arr = np.diff(arr)
            fig = plt.figure()
            plt.hist(arr, bins='auto', density=False, facecolor='g', alpha=0.75)
            plt.ticklabel_format(axis='both',style='sci', scilimits=(0,0))
            plt.title("histogram of: "+serie['title'])
            mean = np.mean(arr)
            stddev = np.std(arr)
            stddev_rel = stddev/mean *100
            plt.text(0.25, 0.75, r'$\mu=' + "{:.3E}".format(mean) + ',\ \sigma_{rel}='+ "{:.3f}".format(stddev_rel) + '\% $', transform=plt.gca().transAxes)
            plt.ylabel('Count')
            plt.xlabel('Ts(s)')
            
            outputdir = PureWindowsPath(sys.argv[2])
            filename = outputdir/PureWindowsPath("ts_bins\\" + serie['title'] + ".png")
            fig.savefig(filename,dpi=600)
            plt.close(fig=fig)

plt.ioff()
scriptdir = sys.argv[1]
scriptdir = PureWindowsPath(scriptdir)

outputdir = sys.argv[2]
outputdir = PureWindowsPath(outputdir + '\\ts_bins')
if not Path(outputdir).is_dir():
    Path(outputdir).mkdir()

filename = Path(scriptdir/PureWindowsPath('data\\ts_bins.json'))
with open(filename,encoding="utf-8") as json_file:
    data = json.load(json_file)
    if __name__ == '__main__':
        with mp.Pool(processes=mp.cpu_count())  as p:
            p.map(serie_single_processor, data)
        
        