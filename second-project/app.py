from flask import Flask
import datetime

app = Flask(__name__)

@app.route('/')
def hello():
    current_time = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return f"""
    <!DOCTYPE html>
    <html>
    <head>
        <title>Python Docker App</title>
        <style>
            body {{
                font-family: Arial, sans-serif;
                max-width: 800px;
                margin: 0 auto;
                padding: 20px;
                background-color: #f0f8ff;
            }}
            .container {{
                background-color: white;
                padding: 30px;
                border-radius: 8px;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            }}
            h1 {{
                color: #2e8b57;
                text-align: center;
            }}
            .time {{
                background-color: #e8f5e8;
                padding: 10px;
                border-radius: 4px;
                text-align: center;
                margin: 20px 0;
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <h1>🐍 Python Docker Environment</h1>
            <p>Welcome to your Python application running in Docker!</p>
            <div class="time">
                <strong>Current server time:</strong> {current_time}
            </div>
            <p>This Flask application is containerized and ready for development.</p>
        </div>
    </body>
    </html>
    """

@app.route('/api/health')
def health():
    return {"status": "healthy", "timestamp": datetime.datetime.now().isoformat()}

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000, debug=True)