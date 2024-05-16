import fastapi
import uvicorn
from fastapi.middleware.cors import CORSMiddleware

app = fastapi.FastAPI()

origins = ["http://localhost:5173"]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/api")
async def root():
    return {"message": "Hello from fastapi!"}


if __name__ == "__main__":
    uvicorn.run(app, port=8008, log_level="info")

# reload_dirs="src-python", reload=True
# uvicorn src-python.main:app --port 8008 --log-level info --reload-dir src-python --reload
