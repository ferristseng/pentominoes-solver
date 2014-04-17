import subprocess 

from flask import Flask
from flask import request, render_template

app = Flask(__name__)

@app.route("/")
def home():
    return render_template('home.html')

def parse_solutions(raw_solutions):
    buf = []
    solutions = []
    for line in raw_solutions:
        if line == '':
            if buf:
                solutions.append(buf)
            buf = []
        else:
            buf.append(line)
    return solutions

@app.route("/solve", methods=['POST'])
def solve():
    opts = {}
    form = request.form
    opts['reflections'] = form.get('reflections', 'false')
    opts['rotations'] = form.get('rotations', 'false')
    opts['filename'] = form.get('filename', '')
    try:
        opts['solutions'] = int(form.get('solutions', '0'))
    except:
        return render_template('solve.html', err="invalid number of solutions")
    p = subprocess.Popen(["./build/main", opts['filename'], 
                        "reflections=%s" % opts['reflections'],
                        "rotations=%s" % opts['rotations'], 
                        "solutions=%s" % opts['solutions']],
                        stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE)
    out, e = p.communicate()
    if e:
        e = e.decode("utf-8").strip()
    out = out.decode("utf-8")
    raw_solutions = out.split('\n')
    solutions = parse_solutions(raw_solutions)
    return render_template('solve.html', solutions=solutions, err=e)

if __name__ == "__main__":
    app.run(debug=True)
