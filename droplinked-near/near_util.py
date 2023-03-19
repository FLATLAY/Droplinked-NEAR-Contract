import requests
import base64
import json
from pygments import highlight, lexers, formatters

rpc_url = "https://archival-rpc.testnet.near.org/"
contract_account_id = "4bb5d093c0c0e1b4874c41216cdabc5ef1c81c5535b25788202f2a8ce145a7d7"

publisher_account = "pub_droplinked.testnet"
producer_account = "prod_droplinked.testnet"
customer_account = "cust_droplinked.testnet"

def b64e(s):
    return base64.b64encode(s.encode()).decode()
def b64d(s):
    return base64.b64decode(s).decode()
def parse_args(args):
    return b64e(json.dumps(args).replace(" ", ""))
def parse_result(res):
    return bytes(res).decode()

def resify (res):
    if res == '[]':
        return []
    return res

def query_contract(entry_point : str, args : dict) -> str :
    body = {
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "query",
        "params": {
            "request_type": "call_function",
            "finality": "final",
            "account_id": contract_account_id,
            "method_name": entry_point,
            "args_base64": parse_args(args)
        }
    }
    try:
        r = requests.post(rpc_url, json=body)
        js = json.loads(r.content.decode())
        result = parse_result(js['result']['result'])
        if [91, 93] == js['result']['result']:
            return '[]'
        else:
            return result
    except : 
        return "{}"

def get_holder(holder_id : int) -> dict:
    return json.loads(query_contract("get_holder", {"holder_id" : holder_id}))

def get_owner_tokens(account_id : str) -> list:
    return json.loads(query_contract("get_owner_tokens", {"account_id" : account_id}))

def get_token_metadata(token_id : int) -> dict:
    return json.loads(query_contract("get_token_metadata", {"token_id" : token_id}))

def get_request(request_id : int) -> dict:
    return json.loads(query_contract("get_request", {"request_id" : request_id}))

def get_publisher_requests(publisher_account : str) -> list:
    return resify(json.loads(query_contract("get_publisher_requests", {"publisher_account" : publisher_account})))
    
def get_producer_requests(producer_account : str) -> list:
    ll = json.loads(query_contract("get_producer_requests", {"producer_account" : producer_account}))
    return resify(ll)

def get_approved(approved_id : int) -> dict:
    return json.loads(query_contract("get_approved", {"approved_id" : approved_id}))

def producers_approved(producer_account : str) -> list:
    return resify(json.loads(query_contract("producers_approved", {"producer_account" : producer_account})))

def publishers_approved(publisher_account : str) -> list:
    return resify(json.loads(query_contract("publishers_approved", {"publisher_account" : publisher_account})))



def return_state():
    producer_holder_ids = get_owner_tokens(producer_account)
    publisher_holder_ids = get_owner_tokens(publisher_account)
    customer_holder_ids = get_owner_tokens(customer_account)
    producer_holders = [{"holder_id" : holder_id , "holder" : get_holder(holder_id)} for holder_id in producer_holder_ids]
    publisher_holders = [{"holder_id" : holder_id , "holder" : get_holder(holder_id)} for holder_id in publisher_holder_ids]
    customer_holders = [{"holder_id" : holder_id , "holder" : get_holder(holder_id)} for holder_id in customer_holder_ids]
    producer_request_ids = json.loads("["+str(get_producer_requests(producer_account))[1:-2]+"]")
    publisher_request_ids = json.loads("["+str(get_publisher_requests(publisher_account))[1:-2]+"]")
    producer_approved_ids = producers_approved(producer_account)
    publisher_approved_ids = publishers_approved(publisher_account)
    producer_approved = [{"approved_id" : approved_id , "approved" : get_approved(approved_id)} for approved_id in producer_approved_ids]
    publisher_approved = [{"approved_id" : approved_id , "approved" : get_approved(approved_id)} for approved_id in publisher_approved_ids]
    return {
        "producer_holders" : producer_holders,
        "publisher_holders" : publisher_holders,
        "customer_holders" : customer_holders,
        "producer_requests" : producer_request_ids,
        "publisher_requests" : publisher_request_ids,
        "producer_approved" : producer_approved,
        "publisher_approved" : publisher_approved
    }

def print_state():
    print(highlight(json.dumps(return_state(), indent=4), lexers.JsonLexer(), formatters.TerminalFormatter()))

if __name__ == "__main__":
    print_state()
