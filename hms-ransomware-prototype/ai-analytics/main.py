
from fastapi import FastAPI
app=FastAPI()
@app.get("/risk")
def risk():
    return {"risk_score":42,"status":"LOW"}
if __name__=="__main__":
    import uvicorn
    uvicorn.run(app,host="0.0.0.0",port=8001)
