class SlugGenerator(object):
    """
    A generator that yields slugs of a given word length forever.
    """
    def __new__(cls, word_length: int) -> SlugGenerator: ...
    def __iter__(self) -> SlugGenerator: ...
    def __next__(self) -> str: ...

def get_slug(word_length: int) -> str:
    """
    Creates a slug of a given word length.
    Args:
        word_length: The length of the slug in words
    """
    ...
def random_slugs(word_length: int, num_outputs: int = 1) -> list[str]:
    """
    Creates a list of random slugs of a given word length.
    Args:
        word_length: The length of the slug in words
        num_outputs: The number of slugs to generate
    """
    ...
def combinations(word_length: int) -> int:
    """
    Calculates the number of possible combinations for a given word length.
    Args:
        word_length: The length of the slug in words
    """
    ...