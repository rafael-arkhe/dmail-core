# Framework de Maturidade - Critérios

Este documento define os 4 níveis de maturidade para as features do projeto.

1. **THEOREM**: O código está completamente implementado e testado em Continuous Integration (CI). É o nível de confiança mais elevado.
2. **LEMMA**: A feature está implementada e passa em testes locais (`cargo test`), mas ainda não tem validação garantida e contínua em CI ou testes exaustivos integrados.
3. **INSPECTED**: A feature ou design teve a fonte primária verificada e inspecionada, porém ainda pode faltar implementação correspondente completa ou cobertura de testes.
4. **CONJECTURE**: A feature foi inferida, desenhada ou documentada (especificação escrita), mas ainda não foi executada ou implementada. Representa um estado de proposta ou intenção não validada em código.
