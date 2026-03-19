FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY main.py .

# Expose the port Uvicorn runs on
EXPOSE 8000

# Run the FastAPI server natively (without the daemon thread wrapper)
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
