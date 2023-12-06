import discord
import random
from discord.ext import commands
import aiohttp
import re
import logging

logging.basicConfig(level=logging.DEBUG)

intents = discord.Intents.default()
intents.members = True

bot = commands.Bot(command_prefix='!', intents=intents)
message_count = 0
target_count = random.randint(25, 250)  # Randomly chosen message count

blacklist_phrases = {'censorlist'}

@bot.event
async def on_ready():
    game = discord.Game("i am robor")
    await bot.change_presence(activity=game)
    print(f'{bot.user.name} is now online and playing {game}')
    
@bot.event
async def on_message(message):
    global message_count, target_count

    if message.author.bot:
        return

    if message.content.startswith(f'<@{bot.user.id}> say '):
        phrase = message.content.split(f'<@{bot.user.id}> say ', 1)[1]
        lower_case_phrase = phrase.lower().replace(' ', '')  # Make the phrase lowercase and remove spaces

        if any(black_word in lower_case_phrase for black_word in blacklist_phrases):
            print('Message contains a blacklisted word.')
            await message.channel.send('nuh uh')
            return
        else:
            await message.channel.send(phrase)
            print('Message is clean.')
            return
        
    # For any other message content
    elif isinstance(message.channel, discord.DMChannel) or f'<@{bot.user.id}>' in message.content:
        async with aiohttp.ClientSession() as session:
            headers = {
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3'}
            async with session.get('https://raw.githubusercontent.com/retardicator/4891/main/list',
                                   headers=headers) as response:
                lines = await response.text()
                line = random.choice(lines.split('\n')).strip()
                print(f'Sending message: {line}')
                await message.channel.send(line)
                print('Message sent')

    message_count += 1
    if message_count >= target_count:
        # Reset message count and choose a new target count
        message_count = 0
        target_count = random.randint(25, 250)

        # Send a randomly selected message from the GitHub link
        async with aiohttp.ClientSession() as session:
            async with session.get('https://raw.githubusercontent.com/retardicator/4891/main/list') as response:
                lines = await response.text()
                messages = lines.split('\n')
                if len(messages) > 0:
                    chosen_message = random.choice(messages).strip()
                    await message.channel.send(chosen_message)

    await bot.process_commands(message)

@bot.command()
async def ping(ctx):
    await ctx.send("Pong!")


bot.run("toker")