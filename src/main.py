#!/usr/bin/env python

import asyncio
import websockets
import pyautogui
import socket
import json


def get_socket() -> str:
    return socket.gethostbyname(socket.gethostname())


HOST: str = get_socket()
PORT: int = 3765


async def hold_for(key: str, duration: int):
    pyautogui.keyDown(key)
    await asyncio.sleep(duration / 1000)
    pyautogui.keyUp(key)


def parse_message(message: str):
    data = json.loads(message)
    command = data["command"]
    key = data["key"]
    if command == "hold":
        duration = data.get("duration")
        return [command, key, duration]
    return [command, key, None]


async def execute(command: str, key: str, duration: int):
    if command == "hold":
        print(key)
    match command:
        case "press":
            if "-" in key:
                [mod, key] = key.split("-")
                with pyautogui.hold(mod):
                    pyautogui.press(key)
            else:
                pyautogui.keyDown(key)
                pyautogui.keyUp(key)
            return
        case "hold":
            if duration == None:
                pyautogui.keyDown(key)
            else:
                await hold_for(key, duration)
            return
        case "release":
            pyautogui.keyUp(key)
            return
        case "*":
            return


async def echo(websocket):
    async for message in websocket:
        [command, key, duration] = parse_message(message)
        asyncio.create_task(execute(command, key, duration))


async def main():
    pyautogui.FAILSAFE = False
    async with websockets.serve(echo, "0.0.0.0", PORT):
        print(f"HOST: localhost, PORT: {PORT}")
        print(f"HOST: 127.0.0.1, PORT: {PORT}")
        print(f"HOST: {HOST} PORT: {PORT}")
        await asyncio.Future()  # run forever


asyncio.run(main())
