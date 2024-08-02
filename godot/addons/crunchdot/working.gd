extends Node

func pack_items(items: Array[Texture]) -> void:
	var crunch := CrunchPacker.new()
	var items_img : Array[Image] = []
	items_img.resize(items.size())
	for item in items.size():
		crunch.add_item(item, items[item].get_width(), items[item].get_height())
	pass
