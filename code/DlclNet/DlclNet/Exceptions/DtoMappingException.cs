namespace DlclNet.Exceptions;

public class DtoMappingException: Exception
{
    private readonly Type? _unmappedType;
    
    public DtoMappingException(string msg, Type? unmappedType = null): base(msg)
    {
        _unmappedType = unmappedType;
    }

    public override string Message
    {
        get
        {
            string message = base.Message;
            if (_unmappedType != null)
                message = $"{message} Type \"{_unmappedType.FullName}\" does not implement \"IDtoDerivable<T>\" and is therefore not mappable to a Dto!";
            return message;
        }
    }
}