import os
from flask import Flask, render_template, request
from mathcore.classical_methods import methods

app = Flask(__name__)


@app.route('/')
def home():
    return render_template('index.html')

@app.route('/calculate', methods=['GET', 'POST'])
def calculate():
    if request.method == 'POST':
        angle = request.form['angle']
        method = request.form['method']
        if angle == '' or method == '':
            return render_template('trifactor2.html')
        angle = float(abs(float(angle)))  # Ensure angle is a float and positive
        if method in methods:
            results = methods[method](angle)
            return render_template('trifactor2.html', results=results)
        else:
            return "Invalid method", 400
    return render_template('trifactor2.html')

if __name__ == '__main__':
    app.run(debug=True)