from django.db import models

# Create your models here.


class Categoria(models.Model):
    nome = models.CharField(max_length=100, null=False, blank=False)
    descricao = models.TextField(null=True, blank=True)

    def __str__(self):
        return self.nome


class Modo():
    CARENCIA = 'CARENCIA'
    NECESSIDADE = 'NECESSIDADE'
    FRAQUEZA = 'FRAQUEZA'
    LIMITE = 'LIMITE'
    

class Item(models.Model):
    nome = models.CharField(max_length=100, null=False, blank=False)
    descricao = models.TextField(null=True, blank=True)
    categoria = models.ForeignKey(Categoria, on_delete=models.CASCADE)
    modo = models.CharField(max_length=100, null=False, blank=False, choices=[(tag, tag.value) for tag in Modo])
    como_estou = models.TextField(null=True, blank=True)
    como_quero_estar = models.TextField(null=True, blank=True)
    o_que_farei = models.TextField(null=True, blank=True)
    quando_farei = models.TextField(null=True, blank=True)

    def __str__(self):
        return self.nome


#     Familia : Lista[dict[str,str]]
#     Social
#     Trabalho
#     Financeiro
#     Vida de Oração
#     Afetividade
#         Humor
#         Emoções
#         Sentimentos
#         Afetos
#         Paixões
#     Sexualidade
#     Saúde
#     Espiritualidade
#     Estudos
#     Lazer
#     Hobbies
#     Amigos
#     Vida Social
#     Vida Profissional
#     Vida Financeira
#     Vida Espiritual
#     Vida Emocional
#     Vida Física
#     Vida Intelectual
#     Vida Familiar
#     Vida Conjugal
#     Vida Social
#     Vida Profissional
#     Vida Financeira
#     Vida Espiritual
#     Vida Emocional
#     Vida Física
#     Vida Intelectual
#     Vida Familiar

