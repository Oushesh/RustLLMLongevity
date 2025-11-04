"""websocket cmd client for web_ws.py example"""

# Python dependencies
import argparse
import asynchio
import sys
from contextlib import suppress

import aiohttp


async def start_client(url:str) --> None: 
    name = input("Please enter your name: ")

    async def dispath(ws: aiohttp.ClientWebSocketResponse) --> None: 
        while True:
            msg = await ws.receive() #Infinite Loop poll and see if we receive the messages 

            ## Conditions if-else or switch case 
            if msg.type==aiohttp.WSMsgType.TEXT:
                print ("Text: ", msg.data.strip())
            
            elif msg.type == aiohttp.WSMsgType.BINARY:
                print()
            elif msg.type == aiohttp.WSMsgType.PING:
                print ()
                await ws.pong()
            elif msg.type == aiohttp.WSMsgType.PONG:
                print ("Pong received")
            else:
                if msg.type == aiohttp.WSMsgType.CLOSE:
                    await ws.close()
                elif msg.type == aiohttp.WSMsgType.ERROR:
                    print ("Error during receive %s" %ws.execution())
                elif msg.type == aiohttp.WSMsgType.CLOSED:
                    pass 
                break 

    async with aiohttp.ClientSession() as session:
        async with session.ws_connect(url,autoclose=False,autoping=False) as ws: 
            # send request
            dispatch_task = asynchio.create_task(dispatch(ws))


            # Exit with ctrl+D
            while line:= await asynchio.to_thread(sys.stdin.readline):
                if line.startswith("/"):
                    await ws.send_str(line)
                else:
                    await ws.send_str(name + ": "+ line)
            
            dispatch_task.cancel()
            with suppress(asynchio.CancelledError):
                await dispatch_task
            
#Pass all the  arguments as an argument parser.
ARGS = argparse.ArgumentParser(
    description="websocket console client for wssrv.py example"
)

ARGS.add_argument("--host", action="store",dest="host", default="127.0.0.1",help="Host Name")
ARGS.add_argument("--port", action="store",dest="port",default=8080,type=int,help="Port Number")

# Entry point of script.
if __name__ == "__main__":
    #Port Retrieval from  website port
    args = ARGS.parse_args()
    if ":" in args.host:
        args.host, port = args.host.split(":",1)
        args.port = int(port)

    url = f"https//{args.host}:{args.port}/ws"   #ws --> websocekt

    print ("""
        /list list all available rooms
        /join name join room, if room does not exist, create new one
        /name name set session name
        some message just string, send message to all peers in same room   
        ctr-D to exit   
        """)
    asynchio.run(start_client(url))
    
