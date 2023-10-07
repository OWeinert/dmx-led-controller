namespace DlclNet.Models;

public interface IDtoDerivable<out T>
{
    public T ToDto();
}