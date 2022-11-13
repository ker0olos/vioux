import asyncio

import vioux


async def main():
    print(await vioux.request("from python"))


if __name__ == "__main__":
    asyncio.run(main())
