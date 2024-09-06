import data.config
from data.auth import auth
from data.stock import get_stock_price



if __name__ == "__main__":
    auth()
    data = get_stock_price()
    print(data)
    
    
    
    