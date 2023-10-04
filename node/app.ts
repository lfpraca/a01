'use strict'

const { PrismaClient } = require('./prisma/db-client')
const prisma = new PrismaClient()
const express = require('express')
const app = express()
const port = 3000

app.get('/tb01', async (_, res) => {
  try {
    const response = await prisma.tb01.findMany({
      take: 10,
      orderBy: {
        col_dt: 'desc',
      },
    })
    res.json(response)
  } catch (e) {
    console.error('Erro ao consultar tb01:', e)
    res.status(500).json({ error: 'Erro ao consultar registros do banco' })
  }
})

app.listen(port, () => {
  console.log(`Servidor iniciado na porta ${port}`)
})
