# Intent Registry — Solana

## El problema
Los agentes en Solana actúan — pero nadie registra 
sus intenciones antes de actuar. No hay forma de 
verificar on-chain si una intención fue declarada 
y cumplida de forma trustless.

## La solución
Intent Registry es un programa en Solana que permite 
registrar y verificar intenciones on-chain. Antes de 
actuar, el agente o usuario declara su intención. 
Cuando la cumple, el programa lo verifica y lo registra 
permanentemente en blockchain.

## ¿Cómo funciona?
1. `register_intent` — Registra una intención on-chain
2. `fulfill_intent` — Marca la intención como cumplida

## Demo en Devnet
- **Program ID:** `J4LA9iU1qVj1ZiXf8F4b55hGfQ6NnNXQfJjquuaFTBo9`
- **TX Register:** `https://explorer.solana.com/tx/4uiHgjBVW4HK1RNXgx3tgXdmwvRxmK64kpEA8Nii1BUvxoQaZHsb7rGNPtLpxYhPiFXNXCBfzCCnG9bNNiVEhHRG?cluster=devnet`
- **TX Fulfill:** `https://explorer.solana.com/tx/25LfgPaDHsakj5Rtx3Wrb99S33qVbN2484dsUy4KUN8L9qSqEusxiHGuDyBb9iwW5kbLGAZiQzjKZrebwkUPPB1i?cluster=devnet`

## Tecnología
- Rust
- Anchor Framework
- Solana Devnet

## Caso de uso — Agent Economy
Intent Registry es infraestructura base para la 
Agent Economy de Solana. Los agentes pueden declarar 
sus intenciones antes de ejecutar acciones, creando 
un historial verificable y trustless de compromisos 
cumplidos on-chain.

## Autor
Anllelo Carreazo
Solana LATAM Hackathon — WayLearn 2026
