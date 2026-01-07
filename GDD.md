\# GDD — Tetris Moderno em Rust (UI moderna, sem assets externos)



\## 1) Visão Geral



\*\*Título:\*\* Tetris Modern

\*\*Gênero:\*\* Puzzle / Arcade

\*\*Plataforma:\*\* Desktop (Windows/macOS/Linux)

\*\*Restrições:\*\*



\* \*\*Sem imagens externas\*\* (PNG/JPG/etc.)

\* \*\*Sem áudio externo\*\* (WAV/MP3/etc.)

\* Tudo gerado por \*\*código\*\* (shapes, cores, gradientes, sombras fake, partículas, textos)



\*\*Objetivo:\*\* Um Tetris com feeling “mobile/console UI”, com interface limpa, suave e moderna, mantendo regras clássicas.



---



\## 2) Stack recomendada (Rust) para UI moderna sem assets



Você tem 3 caminhos bons (todos sem imagens):



\### Opção A — \*\*egui/eframe\*\* (recomendado para “moderno e rápido”)



\* Visual moderno tipo app.

\* Layout fácil: painel lateral (score/next/hold), botões, toggles, tema.

\* Desenho de blocos e animações via `egui` + `epaint`.

\* Ótimo custo/benefício pra um Tetris bonito.



\*\*Crates:\*\* `eframe`, `egui`, `rand`



\### Opção B — \*\*macroquad\*\* (recomendado para “jogo puro, leve e suave”)



\* Render 2D simples e rápido (retângulos, linhas, texto).

\* Animações fáceis, controle de tempo e input direto.

\* Visual fica bem “game”.



\*\*Crates:\*\* `macroquad`, `rand`



\### Opção C — \*\*Bevy\*\* (mais pesado, mas poderoso)



\* ECS, efeitos, animações, pós-processamento.

\* Overkill para Tetris, mas dá um acabamento muito bom.



---



\## 3) Direção de Arte (sem imagens)



\### 3.1 Estilo visual



\*\*Look:\*\* “Neon clean / glassy / minimal”



\* Fundo escuro com leve gradiente

\* Painéis com bordas arredondadas

\* Blocos com:



&nbsp; \* cor sólida + highlight no topo/esquerda

&nbsp; \* sombra suave (fake) por retângulo deslocado

&nbsp; \* borda sutil para separação

\* Grid discreto (linhas bem suaves)



\### 3.2 Cores



\* Paleta moderna:



&nbsp; \* Background: #0B0F1A / #0F172A

&nbsp; \* Painel: #111827

&nbsp; \* Bordas: #1F2937

&nbsp; \* Texto principal: #E5E7EB

&nbsp; \* Texto secundário: #9CA3AF

\* Tetrominoes: 7 cores vivas (sem depender de asset)



\### 3.3 Micro-animações (faz MUITA diferença)



\* \*\*Queda suave:\*\* posição visual interpola (lerp) entre célula anterior e atual (mesmo que a lógica seja grid).

\* \*\*Lock “pop”:\*\* ao travar, um pequeno “scale up -> normal” em 120ms.

\* \*\*Clear line:\*\* fade/flash + shrink horizontal rápido.

\* \*\*Ghost piece:\*\* peça fantasma com alpha baixo.

\* \*\*Hard drop:\*\* rastro (trail) simples desenhado com retângulos translúcidos.



---



\## 4) UX / Layout de Tela



\### 4.1 Layout padrão (16:9 e redimensionável)



\*\*Centro:\*\* tabuleiro

\*\*Direita:\*\* painel com:



\* Score

\* Level

\* Lines

\* Next (peça desenhada em mini-grid 4x4)

\* Hold (opcional)

\* Controles (ícones/texto)

\* Toggle: “Ghost”, “Grid”, “Colorblind”, “Pause”



\*\*Topo:\*\* título e estado (Playing / Paused / Game Over)

\*\*Rodapé:\*\* dicas rápidas (teclas)



\### 4.2 Estados de UI



\* \*\*Main Menu:\*\* Play / Settings / Quit

\* \*\*Playing\*\*

\* \*\*Paused:\*\* overlay blur-ish (fake) com painel no centro

\* \*\*Game Over:\*\* overlay + score final + Restart



---



\## 5) Mecânicas e Regras (clássico com “qualidade moderna”)



\### 5.1 Movimento



\* Left / Right: move 1 célula se possível

\* Soft Drop: aumenta velocidade enquanto segura

\* Hard Drop: cai direto e trava



\### 5.2 Rotação



\* Rotação padrão (90°)

\* \*\*Wall kick simples recomendado\*\* (para ficar “moderno”):



&nbsp; \* se rotacionou e colidiu, tenta offsets: `(±1,0)`, `(±2,0)`, `(0,-1)` antes de desistir

&nbsp; \* isso melhora MUITO a sensação perto das paredes



\### 5.3 Lock delay (moderno)



Para ficar gostoso de jogar:



\* quando toca o chão, não trava instantâneo

\* dá uma pequena janela (ex.: \*\*300ms\*\* ou \*\*X movimentos\*\*) antes do lock final



\### 5.4 Hold (opcional mas “moderno”)



\* Tecla `C` ou `Shift`

\* 1 hold por peça (até travar)



\### 5.5 Ghost Piece (recomendado)



\* Mostra onde a peça vai cair



---



\## 6) Pontuação e Progressão



\### 6.1 Pontuação (padrão moderno simplificado)



\* 1 linha: 100 × (level+1)

\* 2 linhas: 300 × (level+1)

\* 3 linhas: 500 × (level+1)

\* 4 linhas: 800 × (level+1)

\* Soft drop: +1 por célula

\* Hard drop: +2 por célula



\### 6.2 Level e velocidade



\* A cada 10 linhas: `level += 1`

\* Gravidade por level (exemplo):



&nbsp; \* level 0: 800ms

&nbsp; \* level 1: 720ms

&nbsp; \* …

&nbsp; \* mínimo: 80ms



---



\## 7) Input (PC)



\*\*Teclas:\*\*



\* ←/→: mover

\* ↓: soft drop (segurar)

\* Space: hard drop

\* ↑ ou X: rotacionar horário

\* Z: rotacionar anti-horário (opcional)

\* C / Shift: hold (opcional)

\* P / Esc: pause

\* R: restart (em game over)

\* Q: quit



\*\*Gamepad (opcional):\*\*



\* D-pad move

\* A rota

\* B hold

\* Y hard drop



---



\## 8) Renderização (sem assets)



\### 8.1 Como desenhar blocos “bonitos” sem imagem



Cada célula ocupada desenha:



1\. \*\*Sombra\*\*: retângulo escuro (x+2,y+2) com alpha

2\. \*\*Base\*\*: retângulo da cor do tetromino

3\. \*\*Highlight\*\*: retângulo menor no topo/esquerda com cor mais clara

4\. \*\*Borda\*\*: stroke leve



Isso dá aparência “3D suave” sem nenhuma textura.



\### 8.2 Texto moderno



\* egui já fornece fonte; macroquad tem texto básico.

\* Para ficar mais “premium”, use:



&nbsp; \* tamanhos consistentes

&nbsp; \* espaçamento e alinhamento

&nbsp; \* hierarquia: Score grande, labels pequenas



---



\## 9) Arquitetura do Código (sugestão de estrutura)



Se você vier do projeto de terminal, dá pra manter o núcleo e trocar a UI.



\*\*Sugestão (genérica):\*\*



```

tetris_rust/

├── Cargo.toml

└── src/

&nbsp;   ├── main.rs

&nbsp;   ├── app/

&nbsp;   │   ├── mod.rs

&nbsp;   │   ├── state.rs        // menu, playing, paused, gameover

&nbsp;   │   └── settings.rs     // toggles UI (ghost, grid, etc.)

&nbsp;   ├── core/

&nbsp;   │   ├── mod.rs

&nbsp;   │   ├── board.rs

&nbsp;   │   ├── piece.rs

&nbsp;   │   ├── tetromino.rs

&nbsp;   │   ├── rules.rs        // wallkick, lock delay, scoring

&nbsp;   │   └── rng.rs          // bag randomizer (7-bag recomendado)

&nbsp;   ├── render/

&nbsp;   │   ├── mod.rs

&nbsp;   │   ├── theme.rs        // cores, tamanhos, padding

&nbsp;   │   ├── draw\_board.rs

&nbsp;   │   ├── draw\_panels.rs

&nbsp;   │   └── anim.rs         // interpolações/fades

&nbsp;   └── time/

&nbsp;       ├── mod.rs

&nbsp;       └── clock.rs

```



---



\## 10) Features “modernas” que valem ouro (mesmo sem som)



\* \*\*7-bag randomizer\*\* (evita azar extremo)

\* \*\*Ghost piece\*\*

\* \*\*Hold\*\*

\* \*\*Lock delay\*\*

\* \*\*Wall kick simples\*\*

\* \*\*Animação de clear\*\*

\* \*\*Pause overlay bonito\*\*

\* \*\*Tema (Dark/Neon/Minimal)\*\* só mudando cores no código



---



\## 11) Critérios de “Pronto / Completo”



O jogo é considerado completo quando:



\* UI moderna e legível em qualquer resolução

\* Gameplay clássico sólido (colisão, clear lines, game over)

\* Controles fluídos com soft/hard drop

\* Pelo menos: ghost + score + level + next

\* Menu + pause + game over com restart

\* Animações básicas (lock pop + clear)

