from os import getenv

import asyncio
from dotenv import load_dotenv

from aiogram import Bot, Dicpatcher

load_dotenv()
Token=getenv("TOKEN")

dp = Dispatcher()

async def main(token=Token):
    bot = Bot()
    
    await dp.start_polling(bot)


if __name__ == "__main__":
    asyncio.run(main())
