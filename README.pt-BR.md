<p align="center">
  <img src="site/public/favicon.svg" width="72" height="72" alt="Home of Builders">
</p>

<h1 align="center">Home of Builders</h1>

<p align="center">
  Agentes de marketing de elite para o Claude Code e o Codex, e um app de desktop que cria agentes novos para o seu nicho.
</p>

<p align="center">
  <a href="https://github.com/hsnrique/home-of-builders/releases/latest">Baixar</a> ·
  <a href="https://homeofbuilders.com">Site</a> ·
  <a href="README.md">English</a> ·
  <a href="https://buymeacoffee.com/hsnrique">Patrocinar</a>
</p>

---

Escolha um dos 18 agentes base, cruze com uma área (28 nichos e canais) e com as suas necessidades, escolha português ou inglês, e o app pede para o seu Claude Code ou Codex reescrever o agente inteiro para aquele contexto. Você revisa o rascunho, salva na sua biblioteca e instala no Claude Code com um clique.

**Exemplo real:** [`gestor-trafego-implante-odontologico.md`](agents/generated/pt/gestor-trafego-implante-odontologico.md) saiu de `traffic-manager` × *Clínicas e saúde* × "clínica de implante dentário em Curitiba, R$5 mil/mês, leads vão pro WhatsApp", em português, em 74 segundos.

## Os agentes base

Cada agente é um arquivo Markdown em [`agents/core`](agents/core), escrito no mesmo [padrão](STANDARD.md): identidade, missão, princípios, perguntas de briefing, fluxo de trabalho, playbooks, entregáveis, checklist de qualidade, limites e repasses.

| Agente | O que faz |
|---|---|
| [`traffic-manager`](agents/core/traffic-manager.md) | Plano de mídia, estrutura de campanha, teste de criativo e diagnóstico de custo em Meta, Google, TikTok e LinkedIn |
| [`landing-page-optimizer`](agents/core/landing-page-optimizer.md) | Pesquisa de conversão, estrutura de página, confiança no checkout e testes A/B do tamanho do seu tráfego |
| [`launch-strategist`](agents/core/launch-strategist.md) | Calendário de lançamento, pilha de oferta e meta de leads calculada a partir do faturamento |
| [`copywriter`](agents/core/copywriter.md) | Copy de resposta direta na sua voz, a partir de um perfil de voz das suas amostras |
| [`headline-writer`](agents/core/headline-writer.md) | Headlines, ganchos e assuntos de e-mail em volume, pontuados e no limite de cada canal |
| [`email-marketer`](agents/core/email-marketer.md) | Newsletter semanal, sequências, automações e entregabilidade |
| [`video-scriptwriter`](agents/core/video-scriptwriter.md) | Roteiros de Reels, TikTok, anúncios UGC, YouTube e VSL com lista de cenas |
| [`social-media-manager`](agents/core/social-media-manager.md) | Pilares de conteúdo, calendário do mês, legendas e respostas à comunidade |
| [`linkedin-ghostwriter`](agents/core/linkedin-ghostwriter.md) | Posts de LinkedIn na sua voz, prontos para publicar |
| [`x-thread-writer`](agents/core/x-thread-writer.md) | Um tema transformado em thread para o X |
| [`seo-strategist`](agents/core/seo-strategist.md) | Palavras-chave, briefings de conteúdo, correções técnicas e presença nas respostas de IA |
| [`competitor-analyst`](agents/core/competitor-analyst.md) | Bibliotecas de anúncios, ofertas, reclamações e brechas de posicionamento num battlecard |
| [`market-researcher`](agents/core/market-researcher.md) | Pesquisa com fonte e nível de confiança, roteiro de entrevista e tamanho de mercado |
| [`marketing-analyst`](agents/core/marketing-analyst.md) | Lê as suas exportações e entrega o relatório no seu formato |
| [`brand-strategist`](agents/core/brand-strategist.md) | Posicionamento, mensagens e guia de voz com exemplos reais |
| [`sdr`](agents/core/sdr.md) | Qualifica leads, escreve sequências de prospecção e marca reuniões |
| [`sales-assistant`](agents/core/sales-assistant.md) | Acompanha o lead até o fechamento, respondendo com o seu próprio FAQ |
| [`negotiation-specialist`](agents/core/negotiation-specialist.md) | One-sheet de negociação, perguntas calibradas e plano de ofertas com números exatos |

Os agentes base são escritos em inglês. Para ter qualquer um deles em português, gere pelo app com o idioma Português.

Os playbooks aplicam os frameworks dos livros de referência de cada área e dizem de onde vêm: *Breakthrough Advertising*, *Influence*, *Tested Advertising Methods*, *Never Split the Difference*, *Getting to Yes*, *Positioning*, *Obviously Awesome*, *How Brands Grow*, *StoryBrand*, *The Mom Test*, *Contagious*, *Hook Point*, *Don't Make Me Think*, *Predictable Revenue*, *Lean Analytics*, *Everybody Writes* e outros. Os agentes parafraseiam e aplicam as ideias; não contêm trechos dos livros.

## Instalação

1. Instale o [Claude Code](https://docs.claude.com/en/docs/claude-code/overview) ou o [Codex](https://github.com/openai/codex) e faça login uma vez.
2. Pegue o app:
   - **macOS:** baixe o `.dmg` em [Releases](https://github.com/hsnrique/home-of-builders/releases/latest). Os builds ainda não são notarizados: na primeira vez, abra o app, vá em **Ajustes do Sistema → Privacidade e Segurança** e clique em **Abrir Mesmo Assim**.
   - **Windows e Linux:** [rode a partir do código](#rodar-a-partir-do-código).
3. Abra. A biblioteca é criada em `Documentos/Home of Builders` com todos os agentes base.

Só quer os agentes? Copie qualquer arquivo de [`agents/`](agents) para `~/.claude/agents/` e o Claude Code passa a usar. O Codex lê agentes em TOML: use **Instalar no → Codex** no app.

## Como a geração funciona

- O app roda `claude -p --safe-mode --tools ""` ou `codex exec --sandbox read-only` na sua máquina, o que você escolher em Configurações, com o seu próprio login: sem chave de API e sem custo além do seu plano.
- `--tools ""` faz o modelo só responder com texto; ele não lê nem altera seus arquivos.
- `--safe-mode` deixa o seu `CLAUDE.md`, hooks e plugins fora da geração, então o agente sai igual para todo mundo.
- Nada mais sai do seu computador. O app não tem telemetria nem conta.

## Estrutura da biblioteca

```
agents/core/              agentes base, em inglês
agents/generated/en|pt/   agentes que você cria, por idioma
catalog/areas.json        nichos e canais para cruzar
STANDARD.md               o padrão que todo agente segue
app/                      app de desktop (Tauri 2, TypeScript, Rust)
site/                     homeofbuilders.com (Vite)
```

Para adicionar uma área, edite `catalog/areas.json`. Para adicionar um agente base, coloque um arquivo em `agents/core/` seguindo o [STANDARD.md](STANDARD.md). O app enxerga os dois na próxima vez que abrir.

## Rodar a partir do código

Precisa de Node 24, Rust stable e Claude Code ou Codex.

```bash
cd app
npm ci
npm run tauri dev      # desenvolvimento
npm run tauri build    # gera o instalador do seu sistema
```

Testes: `npm test`, `npm run build` e `cargo test` (dentro de `app/src-tauri`).

## Contribuir

Agentes novos, áreas, traduções e correções são bem-vindos. Comece pelo [CONTRIBUTING.md](CONTRIBUTING.md).

## Patrocinar

O Home of Builders é grátis e continua grátis. Se ele te poupou uma contratação ou um fim de semana, [me pague um café](https://buymeacoffee.com/hsnrique): isso paga agentes novos e mantém o app em dia com o Claude Code e o Codex.

## Créditos

Criado e mantido por **Henrique Braga** ([@hsnrique](https://github.com/hsnrique), [hsnrique.com](https://hsnrique.com)).

Licença [MIT](LICENSE).
