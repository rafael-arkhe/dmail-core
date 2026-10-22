# WormGraph MMR Profile

Este documento especifica o perfil do MMR (Merkle Mountain Range) utilizado no ecossistema Arkhe, nomeado como **WormGraph MMR**. A configuração do MMR obedece a 7 pontos fundamentais de design para garantir a determinística, segurança criptográfica e separação de domínios em provas portáteis offline.

## Os 7 Pontos do Perfil

1. **Domain Separation (Separação de Domínios)**
   As operações de hash no MMR utilizam bytes de prefixo distintos para separar semanticamente cada tipo de componente criptográfico, evitando colisões indesejadas:
   - `0x00`: Folha (Leaf)
   - `0x01`: Nó Interno (Node)
   - `0x02`: Raiz (Root / Checkpoint)
   - `0x03`: Bag (combinação dos peaks)

2. **Peak Ordering (Ordenação dos Peaks)**
   Quando é necessário recuperar ou agregar os peaks (as raízes das árvores binárias perfeitas contidas no MMR), eles são iterados e armazenados em ordem **da esquerda para a direita** (do maior pico para o menor), o que está diretamente alinhado com o índice e o peso de cada sub-árvore na floresta.

3. **Bagging Direction (Direção do Bagging)**
   A redução final (bagging) dos peaks para derivar a raiz única do MMR processa-se da **direita para a esquerda** (right-to-left).
   Isso significa que o agregador começa com os menores picos (ou picos vazios, ver ponto 4 e 5) e acumula progressivamente subindo em direção ao maior pico, com correções aplicadas ao comentário ou método `rev()` contraditório.

4. **Accumulator Initialization (Inicialização do Acumulador de Bagging)**
   A operação de `hash_bag` requer um estado base claro ao combinar os peaks. O acumulador deve sinalizar de forma explícita o momento em que a árvore (ou o peak) inicia a combinação, resolvendo as ambiguidades e os "comprimentos variáveis" nas folhas, possivelmente integrando um marcador `acc=None` mapeado para um estado nulo `0x00` ou `0x01` explícito.

5. **Empty Tree Representation (Árvore Vazia)**
   O hash que representa a árvore vazia tem que ser deterministicamente seguro e inconfundível. Não utilizamos array nulo (zeroed bytes); a representação formal é acordada previamente ou através do próprio estado de `acc=None` ou de uma string prefixada hashada correspondente a "EmptyMMR".

6. **Inclusion Proof Format (Formato da Prova de Inclusão)**
   As provas (`PortableInclusionProof`) encapsulam a posição folha (índice MMR), o hash original provado e o caminho autêntico (`peaks` ou `siblings` da sub-árvore e os picos necessários para completar o bagging) num layout canônico serializado para que possa ser avaliado *offline* (em custo O(log n)) sem recomputar todo o estado.

7. **Consistency Proof Format (Formato da Prova de Consistência)**
   As provas de consistência verificam append-only: a validação criptográfica de que um novo estado MMR estende o estado anterior perfeitamente. Contém essencialmente a contagem inicial, contagem final, e um vetor mínimo de "vértices de correção" que permite derivar ambos os baggings provando que nenhum dado pré-existente foi corrompido ou removido.
