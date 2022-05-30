class Resource:
	__slots__ = ("_ptr",)
	DTOR = None

	def __init__(self, ptr: int) -> None:
		self._ptr = ptr

	@property
	def ptr(self) -> int:
		if self._ptr is None:
			raise RuntimeError("smoltcp " + self.__class__.__name__ + " was freed")
		return self._ptr

	def free(self):
		if self._ptr:
			self.__class__.DTOR(self._ptr)
			self._ptr = None


class ResourceWithParent(Resource):
	__slots__ = ()

	CTOR = None

	def __init__(self, parent: Resource) -> None:
		self.parent = parent
		self.ptr = self.__class__.CTOR(parent.ptr)
